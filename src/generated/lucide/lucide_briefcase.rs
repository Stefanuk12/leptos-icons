use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" ></ path > < rect height = "14" y = "6" x = "2" rx = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;