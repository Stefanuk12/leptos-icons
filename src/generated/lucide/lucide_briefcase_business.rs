use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12h.01" ></ path > < path d = "M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" ></ path > < path d = "M22 13a18.15 18.15 0 0 1-20 0" ></ path > < rect x = "2" width = "20" height = "14" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE_BUSINESS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;