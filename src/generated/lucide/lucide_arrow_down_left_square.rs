use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 8-8 8" ></ path > < path d = "M16 16H8V8" ></ path > < / > } } pub const LucideArrowDownLeftSquare : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;