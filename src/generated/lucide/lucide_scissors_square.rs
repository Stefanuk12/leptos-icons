use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "20" x = "2" rx = "2" y = "2" ></ rect > < circle cy = "8" cx = "8" r = "2" ></ circle > < path d = "M9.414 9.414 12 12" ></ path > < path d = "M14.8 14.8 18 18" ></ path > < circle r = "2" cx = "8" cy = "16" ></ circle > < path d = "m18 6-8.586 8.586" ></ path > < / > } } pub const LucideScissorsSquare : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;