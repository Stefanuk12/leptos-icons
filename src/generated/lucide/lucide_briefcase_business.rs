use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12h.01" ></ path > < path d = "M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" ></ path > < path d = "M22 13a18.15 18.15 0 0 1-20 0" ></ path > < rect y = "6" width = "20" rx = "2" height = "14" x = "2" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE_BUSINESS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;