use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" ></ path > < rect rx = "2" x = "2" y = "6" width = "20" height = "14" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;