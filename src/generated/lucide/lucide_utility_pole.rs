use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v20" ></ path > < path d = "M2 5h20" ></ path > < path d = "M3 3v2" ></ path > < path d = "M7 3v2" ></ path > < path d = "M17 3v2" ></ path > < path d = "M21 3v2" ></ path > < path d = "m19 5-7 7-7-7" ></ path > < / > } } pub const LUCIDE_UTILITY_POLE : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;