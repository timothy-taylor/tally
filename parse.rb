# frozen_string_literal: true

class Tally
  def initialize
    @new = get_tasks_and_times("logs.txt",2)
    @old = get_tasks_and_times("tally.txt",1)
    open_and_update
    clean_up
  end

  def clean_up
    system('cat tally.txt')
    File.delete("logs.txt")
    system('touch logs.txt')
  end

  def open_and_update
    lines = @new.each do |k,v|
      if @old.keys.include?(k)
        @old[k] += v
      else
        @old[k] = v
      end
    end

    File.open("tally.txt", 'w') do |file|
      file.puts lines
    end
  end

  def get_tasks_and_times(path, ix)
    task_and_times = {}
    IO.foreach(path){ |log|
      words = log.split
      task = words[0]
      time = words[ix].to_i
      if task_and_times.keys.include?(task)
        task_and_times[task] += time
      else 
        task_and_times[task] = time
      end
    }
    task_and_times
  end

  def totals
    let total = @old.fetch_values().sum()
    @old.each do |k,v|
      puts "#{k}: #{(total / v) * 100}%"
    end
  end
end

tally = Tally.new
tally.totals
