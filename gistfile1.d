string string_prompt(Window parent, string prompt, string initialValue = null)
{
    string result;
    auto window = new Window(parent, "Prompt");
    auto label = new Label(window, prompt);
    auto input = new Entry(window);
    auto okButton = new Button(window, "OK");
    auto cancelButton = new Button(window, "Cancel");
    
    void close(CommandArgs _ = CommandArgs.init)
    {
        window.setGrab(false);
        window.destroy;
    }
    
    void close_with_value(CommandArgs _)
    {
        result = input.getValue;
        
        close;
    }
    
    okButton.setCommand(&close_with_value);
    cancelButton.setCommand(&close);
    input.bind("<Return>", &close_with_value);
    label.grid(0, 0, 5, 0, 2);
    input.grid(0, 1, 5, 0, 2);
    okButton.grid(0, 2, 5);
    cancelButton.grid(1, 2, 5);
    input.setValue(initialValue);
    input.focus;
    window.setGrab(true);
    window.wait;
    
    return result;
}