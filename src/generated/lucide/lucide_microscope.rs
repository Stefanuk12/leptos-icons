use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 18h8" ></ path > < path d = "M3 22h18" ></ path > < path d = "M14 22a7 7 0 1 0 0-14h-1" ></ path > < path d = "M9 14h2" ></ path > < path d = "M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z" ></ path > < path d = "M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3" ></ path > < / > } } pub const LUCIDE_MICROSCOPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;