use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.5 3h15" ></ path > < path d = "M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3" ></ path > < path d = "M6 14h12" ></ path > < / > } } pub const LUCIDE_BEAKER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;