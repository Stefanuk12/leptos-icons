use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.8 4A6.3 8.7 0 0 1 20 9" ></ path > < path d = "M9 9h.01" ></ path > < circle r = "7" cx = "9" cy = "9" ></ circle > < rect width = "10" x = "4" rx = "2" height = "6" y = "16" ></ rect > < path d = "M14 19c3 0 4.6-1.6 4.6-1.6" ></ path > < circle cy = "16" r = "2" cx = "20" ></ circle > < / > } } pub const LUCIDE_BELL_ELECTRIC : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round")] } ;