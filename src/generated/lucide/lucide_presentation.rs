use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3h20" ></ path > < path d = "M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3" ></ path > < path d = "m7 21 5-5 5 5" ></ path > < / > } } pub const LUCIDE_PRESENTATION : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;