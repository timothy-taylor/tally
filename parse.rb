# frozen_string_literal: true

class Tally
  def initialize
    @new = get_tasks_and_times("logs.txt",2)
    @old = get_tally("tally.txt",1)
  end

  def clean_up
    File.delete("logs.txt")
    system('touch logs.txt')
  end

  def open_and_update
    @new.each do |k,v|
      if @old.keys.include?(k)
        @old[k] += v
      else
        @old[k] = v
      end
    end
  
    @old["total"] = totals

    File.open("tally.txt", 'w') do |file|
      file.puts @old
    end
  end

  def get_tally(path,ix)
    tally = {}
    IO.foreach(path) do |log|
      arr = log.split
      arr.each do |e|
        a = e.gsub(/[^a-z0-9\s]/i, ' ').strip.split
        task = a[0]
        time = a[ix].to_i
        tally[task] = time
      end
    end
    tally
  end

  def get_tasks_and_times(path, ix)
    task_and_times = {}
    IO.foreach(path) do |log|
      words = log.split
      task = words[0]
      time = words[ix].to_i
      if task_and_times.keys.include?(task)
        task_and_times[task] += time
      else 
        task_and_times[task] = time
      end
    end
    task_and_times
  end

  def totals
    @old.except("total").values.sum
  end

  def print_totals
    puts "Since Dec 6, 2021"
    total = totals
    puts "Total: #{total / 60} hours, #{total % 60} minutes"
    puts "//////////////////"
    @old.except("total").each do |k,v|
      perc = (v.to_f / total.to_f) * 100.0
      puts "#{k}: #{perc.round(1)}%; #{v / 60} hours, #{v % 60} minutes"
    end
  end
end

tally = Tally.new
tally.open_and_update
tally.print_totals
tally.clean_up
