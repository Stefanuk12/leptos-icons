use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.8 4A6.3 8.7 0 0 1 20 9" ></ path > < path d = "M9 9h.01" ></ path > < circle cy = "9" r = "7" cx = "9" ></ circle > < rect y = "16" height = "6" rx = "2" x = "4" width = "10" ></ rect > < path d = "M14 19c3 0 4.6-1.6 4.6-1.6" ></ path > < circle cx = "20" cy = "16" r = "2" ></ circle > < / > } } pub const LUCIDE_BELL_ELECTRIC : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;