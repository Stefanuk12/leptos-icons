use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" y = "4" height = "16" x = "2" rx = "2" ></ rect > < circle cy = "10" r = "2" cx = "8" ></ circle > < path d = "M8 12h8" ></ path > < circle r = "2" cy = "10" cx = "16" ></ circle > < path d = "m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" ></ path > < / > } } pub const LUCIDE_CASSETTE_TAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;