use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "20" height = "20" x = "2" rx = "2" ></ rect > < circle cx = "8" cy = "8" r = "2" ></ circle > < path d = "M9.414 9.414 12 12" ></ path > < path d = "M14.8 14.8 18 18" ></ path > < circle cx = "8" cy = "16" r = "2" ></ circle > < path d = "m18 6-8.586 8.586" ></ path > < / > } } pub const LUCIDE_SQUARE_SCISSORS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;