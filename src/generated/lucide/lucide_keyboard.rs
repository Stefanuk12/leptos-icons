use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8h.001" ></ path > < path d = "M10 8h.001" ></ path > < path d = "M14 8h.001" ></ path > < path d = "M18 8h.001" ></ path > < path d = "M8 12h.001" ></ path > < path d = "M12 12h.001" ></ path > < path d = "M16 12h.001" ></ path > < path d = "M7 16h10" ></ path > < / > } } pub const LucideKeyboard : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;