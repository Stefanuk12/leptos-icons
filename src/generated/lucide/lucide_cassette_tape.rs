use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" x = "2" y = "4" rx = "2" width = "20" ></ rect > < circle cy = "10" cx = "8" r = "2" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "10" r = "2" ></ circle > < path d = "m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" ></ path > < / > } } pub const LUCIDE_CASSETTE_TAPE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;