use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 20v2" ></ path > < path d = "M14 20v2" ></ path > < path d = "M18 20v2" ></ path > < path d = "M21 20H3" ></ path > < path d = "M6 20v2" ></ path > < path d = "M8 16V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v12" ></ path > < rect x = "4" width = "16" height = "10" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_BRIEFCASE_CONVEYOR_BELT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;