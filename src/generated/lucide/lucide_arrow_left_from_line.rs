use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 6-6 6 6 6" ></ path > < path d = "M3 12h14" ></ path > < path d = "M21 19V5" ></ path > < / > } } pub const LucideArrowLeftFromLine : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;