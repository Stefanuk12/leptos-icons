use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 3h12l4 6-10 13L2 9Z" ></ path > < path d = "M11 3 8 9l4 13 4-13-3-6" ></ path > < path d = "M2 9h20" ></ path > < / > } } pub const LUCIDE_GEM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;