use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12.5 17-.5-1-.5 1h1z" ></ path > < path d = "M15 22a1 1 0 0 0 1-1v-1a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20v1a1 1 0 0 0 1 1z" ></ path > < circle cx = "15" cy = "12" r = "1" ></ circle > < circle cy = "12" r = "1" cx = "9" ></ circle > < / > } } pub const LUCIDE_SKULL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;