use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12A10 10 0 1 1 12 2" ></ path > < path d = "M22 2 12 12" ></ path > < path d = "M16 2h6v6" ></ path > < / > } } pub const LucideArrowUpRightFromCircle : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;