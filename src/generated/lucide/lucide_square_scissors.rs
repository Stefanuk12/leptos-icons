use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" y = "2" rx = "2" width = "20" x = "2" ></ rect > < circle r = "2" cx = "8" cy = "8" ></ circle > < path d = "M9.414 9.414 12 12" ></ path > < path d = "M14.8 14.8 18 18" ></ path > < circle cx = "8" r = "2" cy = "16" ></ circle > < path d = "m18 6-8.586 8.586" ></ path > < / > } } pub const LUCIDE_SQUARE_SCISSORS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;