use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < circle cy = "10" cx = "12" r = "3" ></ circle > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "m14 12.5 1 5.5-3-1-3 1 1-5.5" ></ path > < / > } } pub const LUCIDE_FILE_BADGE_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;