use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 3h12l4 6-10 13L2 9Z" ></ path > < path d = "M11 3 8 9l4 13 4-13-3-6" ></ path > < path d = "M2 9h20" ></ path > < / > } } pub const LUCIDE_GEM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;