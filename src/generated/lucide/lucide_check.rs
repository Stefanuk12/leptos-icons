use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 6 9 17l-5-5" ></ path > < / > } } pub const LucideCheck : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;