use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "20" rx = "2" height = "16" y = "4" ></ rect > < circle cx = "8" r = "2" cy = "10" ></ circle > < path d = "M8 12h8" ></ path > < circle cy = "10" cx = "16" r = "2" ></ circle > < path d = "m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" ></ path > < / > } } pub const LUCIDE_CASSETTE_TAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;