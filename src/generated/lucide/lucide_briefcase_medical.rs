use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 11v4" ></ path > < path d = "M14 13h-4" ></ path > < path d = "M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" ></ path > < path d = "M18 6v14" ></ path > < path d = "M6 6v14" ></ path > < rect x = "2" y = "6" height = "14" rx = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE_MEDICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;