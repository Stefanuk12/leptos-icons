use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 11H4" ></ path > < path d = "M20 7H4" ></ path > < path d = "M7 21V4a1 1 0 0 1 1-1h4a1 1 0 0 1 0 12H7" ></ path > < / > } } pub const LUCIDE_PHILIPPINE_PESO : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;