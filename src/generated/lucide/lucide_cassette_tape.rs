use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "16" rx = "2" x = "2" y = "4" ></ rect > < circle cx = "8" r = "2" cy = "10" ></ circle > < path d = "M8 12h8" ></ path > < circle r = "2" cx = "16" cy = "10" ></ circle > < path d = "m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" ></ path > < / > } } pub const LUCIDE_CASSETTE_TAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2")] } ;