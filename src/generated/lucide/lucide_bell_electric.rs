use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.8 4A6.3 8.7 0 0 1 20 9" ></ path > < path d = "M9 9h.01" ></ path > < circle r = "7" cx = "9" cy = "9" ></ circle > < rect x = "4" y = "16" width = "10" rx = "2" height = "6" ></ rect > < path d = "M14 19c3 0 4.6-1.6 4.6-1.6" ></ path > < circle cy = "16" r = "2" cx = "20" ></ circle > < / > } } pub const LUCIDE_BELL_ELECTRIC : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round")] } ;