use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cx = "11.5" cy = "14.5" r = "2.5" ></ circle > < path d = "M13.3 16.3 15 18" ></ path > < / > } } pub const LUCIDE_FILE_SEARCH_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;