use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "9" r = "1" ></ circle > < circle cx = "15" cy = "12" r = "1" ></ circle > < path d = "M8 20v2h8v-2" ></ path > < path d = "m12.5 17-.5-1-.5 1h1z" ></ path > < path d = "M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20" ></ path > < / > } } pub const LUCIDE_SKULL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;