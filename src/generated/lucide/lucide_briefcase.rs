use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" ></ path > < rect rx = "2" x = "2" width = "20" y = "6" height = "14" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;