use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" width = "18" height = "18" ></ rect > < circle fill = "currentColor" r = ".5" cy = "7.5" cx = "7.5" ></ circle > < path d = "m7.9 7.9 2.7 2.7" ></ path > < circle r = ".5" cx = "16.5" fill = "currentColor" cy = "7.5" ></ circle > < path d = "m13.4 10.6 2.7-2.7" ></ path > < circle cy = "16.5" r = ".5" fill = "currentColor" cx = "7.5" ></ circle > < path d = "m7.9 16.1 2.7-2.7" ></ path > < circle cx = "16.5" fill = "currentColor" r = ".5" cy = "16.5" ></ circle > < path d = "m13.4 13.4 2.7 2.7" ></ path > < circle cx = "12" r = "2" cy = "12" ></ circle > < / > } } pub const LUCIDE_VAULT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;