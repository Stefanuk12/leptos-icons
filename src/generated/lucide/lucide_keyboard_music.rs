use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8h4" ></ path > < path d = "M14 8h.01" ></ path > < path d = "M18 8h.01" ></ path > < path d = "M2 12h20" ></ path > < path d = "M6 12v4" ></ path > < path d = "M10 12v4" ></ path > < path d = "M14 12v4" ></ path > < path d = "M18 12v4" ></ path > < / > } } pub const LucideKeyboardMusic : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;