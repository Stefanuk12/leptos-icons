use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 3h12l4 6-10 13L2 9Z" ></ path > < path d = "M11 3 8 9l4 13 4-13-3-6" ></ path > < path d = "M2 9h20" ></ path > < / > } } pub const LucideGem : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;