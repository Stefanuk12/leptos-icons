use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 11v4" ></ path > < path d = "M14 13h-4" ></ path > < path d = "M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" ></ path > < path d = "M18 6v14" ></ path > < path d = "M6 6v14" ></ path > < rect y = "6" width = "20" rx = "2" height = "14" x = "2" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE_MEDICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;