use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ></ rect > < circle cy = "7.5" r = ".5" cx = "7.5" fill = "currentColor" ></ circle > < path d = "m7.9 7.9 2.7 2.7" ></ path > < circle fill = "currentColor" cy = "7.5" cx = "16.5" r = ".5" ></ circle > < path d = "m13.4 10.6 2.7-2.7" ></ path > < circle r = ".5" fill = "currentColor" cy = "16.5" cx = "7.5" ></ circle > < path d = "m7.9 16.1 2.7-2.7" ></ path > < circle r = ".5" cx = "16.5" cy = "16.5" fill = "currentColor" ></ circle > < path d = "m13.4 13.4 2.7 2.7" ></ path > < circle r = "2" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_VAULT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;