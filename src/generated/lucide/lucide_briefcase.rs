use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" ></ path > < rect y = "6" x = "2" height = "14" rx = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;