use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 21s-4-3-4-9 4-9 4-9" ></ path > < path d = "M16 3s4 3 4 9-4 9-4 9" ></ path > < / > } } pub const LucideParentheses : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;