use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M15 18a3 3 0 1 0-6 0" ></ path > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z" ></ path > < circle r = "2" cy = "13" cx = "12" ></ circle > < / > } } pub const LUCIDE_FILE_USER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;