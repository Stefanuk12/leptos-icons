use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5" ></ path > < circle cy = "12" r = "2" cx = "13" ></ circle > < path d = "M18 19c-2.8 0-5-2.2-5-5v8" ></ path > < circle r = "2" cx = "20" cy = "19" ></ circle > < / > } } pub const LUCIDE_FOLDER_GIT_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;