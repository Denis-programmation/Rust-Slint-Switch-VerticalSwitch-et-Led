slint::slint!
{
    #[edition = "1.19"]
    
    import { Switch, VerticalBox } from "std-widgets.slint";
        
    export component Fenetre inherits Window
    {
        width: 200px;
        height: 200px;
        
        callback envoi_etat1(bool);
        callback envoi_etat2(bool);
        
        VerticalBox
        {
            padding-bottom:10px;
            alignment: center;
            
            Switch
            {
                checked: false;
                text: 1;
                toggled => 
                {
                    envoi_etat1(self.checked);
                }
            }

            Switch
            {
                checked: false;
                text: 2;
                toggled => 
                {
                    envoi_etat2(self.checked);
                }
            }

        }
    }
}

fn main()
{
    let fenetre = Fenetre::new().unwrap();

    fenetre.on_envoi_etat1 ( | actif | { println!( "Switch 1 activé {actif}" ) } );
    fenetre.on_envoi_etat2 ( | actif | { println!( "Switch 2 activé {actif}" ) } );
        
    fenetre.run().unwrap();
}
