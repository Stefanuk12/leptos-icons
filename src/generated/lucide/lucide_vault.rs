use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" x = "3" y = "3" ></ rect > < circle cx = "7.5" fill = "currentColor" r = ".5" cy = "7.5" ></ circle > < path d = "m7.9 7.9 2.7 2.7" ></ path > < circle cx = "16.5" cy = "7.5" r = ".5" fill = "currentColor" ></ circle > < path d = "m13.4 10.6 2.7-2.7" ></ path > < circle fill = "currentColor" cx = "7.5" cy = "16.5" r = ".5" ></ circle > < path d = "m7.9 16.1 2.7-2.7" ></ path > < circle cx = "16.5" cy = "16.5" r = ".5" fill = "currentColor" ></ circle > < path d = "m13.4 13.4 2.7 2.7" ></ path > < circle cx = "12" cy = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_VAULT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;