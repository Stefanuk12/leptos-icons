use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse ry = "3" rx = "9" cx = "12" cy = "5" ></ ellipse > < path d = "M3 12a9 3 0 0 0 5 2.69" ></ path > < path d = "M21 9.3V5" ></ path > < path d = "M3 5v14a9 3 0 0 0 6.47 2.88" ></ path > < path d = "M12 12v4h4" ></ path > < path d = "M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16" ></ path > < / > } } pub const LUCIDE_DATABASE_BACKUP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;