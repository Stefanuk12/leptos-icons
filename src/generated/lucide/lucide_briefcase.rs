use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" ></ path > < rect y = "6" width = "20" x = "2" height = "14" rx = "2" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;