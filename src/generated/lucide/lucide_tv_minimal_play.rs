use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 7.75a.75.75 0 0 1 1.142-.638l3.664 2.249a.75.75 0 0 1 0 1.278l-3.664 2.25a.75.75 0 0 1-1.142-.64z" ></ path > < path d = "M7 21h10" ></ path > < rect width = "20" rx = "2" x = "2" height = "14" y = "3" ></ rect > < / > } } pub const LUCIDE_TV_MINIMAL_PLAY : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;