use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 4v16" ></ path > < path d = "M17 4v16" ></ path > < path d = "M19 4H9.5a4.5 4.5 0 0 0 0 9H13" ></ path > < / > } } pub const LUCIDE_PILCROW : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;