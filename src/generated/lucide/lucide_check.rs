use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 6 9 17l-5-5" ></ path > < / > } } pub const LucideCheck : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;