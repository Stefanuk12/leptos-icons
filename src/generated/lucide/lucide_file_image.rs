use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cy = "12" r = "2" cx = "10" ></ circle > < path d = "m20 17-1.296-1.296a2.41 2.41 0 0 0-3.408 0L9 22" ></ path > < / > } } pub const LUCIDE_FILE_IMAGE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;