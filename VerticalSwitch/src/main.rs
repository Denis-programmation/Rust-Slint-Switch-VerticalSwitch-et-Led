slint::slint!
{
    #[edition = "1.19"]

    export component Led inherits Rectangle
    {
        in property <bool> allume: false;

        width: 30px;
        height: 30px;
        border-radius: 15px;
        background: allume ? Colors.red : #888888;

        Rectangle 
        {
            width: 28px;
            height: 28px;
            border-radius: 14px;
            x: 1px;
            y: 1px;
        }

    }
    
    export component VerticalSwitch inherits Rectangle
    {
        in-out property <bool> checked: false;

        callback toggled(bool);

        width: 30px;
        height: 70px;
        border-radius: 15px;
        background: checked ? #4caf50 : #888888;

        // Le curseur monte quand le switch est activé.
        Rectangle 
        {
            width: 20px;
            height: 30px;
            border-radius: 10px;
            x: 5px;
            y: checked ? 5px : 36px;
            background: white;
        }

        TouchArea 
        {
            clicked => 
            {
                root.checked = !root.checked;
                root.toggled(root.checked);
            }
        }

    }

    import { HorizontalBox, VerticalBox } from "std-widgets.slint";
     
    export component Fenetre inherits Window 
    {
        width: 200px;
        height: 200px;

        in property <bool> led1: false;
        
        callback toggled1(bool);
        callback toggled2(bool);
        callback toggled3(bool);
        callback toggled4(bool);

        VerticalBox
        {
            HorizontalBox
            {
                Led
                {
                    allume <=> led1;
                }

                led2 := Led
                {
                    allume: false;
                }

                led3 := Led
                {
                    allume: false;
                }

                led4 := Led
                {
                    allume: false;
                }

            }

            HorizontalBox
            {
                vs1 := VerticalSwitch
                {
                    checked: false;
                    toggled =>
                    {
                        root.toggled1(self.checked);
//                        led1.allume = self.checked;
                    }
                }

                vs2 := VerticalSwitch
                {
                    checked: false;
                    toggled =>
                    {
                        root.toggled2(self.checked);
                        led2.allume = self.checked;
                    }
                }

                vs3 := VerticalSwitch
                {
                    checked: false;
                    toggled =>
                    {
                        root.toggled3(self.checked);
                        led3.allume = self.checked;
                    }
                }

                vs4 := VerticalSwitch
                {
                    checked: false;
                    toggled =>
                    {
                        root.toggled4(self.checked);
                        led4.allume = self.checked;
                    }
                }

            }
        }
    }
}

fn main()
{
    let fenetre = Fenetre::new().unwrap();
    let fenetre2 = fenetre.clone_strong();

    fenetre.on_toggled1 ( move | actif | { 
                                                fenetre2.set_led1 ( actif );
                                                println!( "Switch 1 activé {actif}" )
                                               } );
    fenetre.on_toggled2 ( | actif | { println!( "Switch 2 activé {actif}" ) } );
    fenetre.on_toggled3 ( | actif | { println!( "Switch 3 activé {actif}" ) } );
    fenetre.on_toggled4 ( | actif | { println!( "Switch 4 activé {actif}" ) } );
        
    fenetre.run().unwrap();
}




