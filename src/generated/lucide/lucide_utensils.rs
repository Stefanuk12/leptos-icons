use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2" ></ path > < path d = "M7 2v20" ></ path > < path d = "M21 15V2a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7" ></ path > < / > } } pub const LUCIDE_UTENSILS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;