use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" ></ path > < path d = "M14 15H9v-5" ></ path > < path d = "M16 3h5v5" ></ path > < path d = "M21 3 9 15" ></ path > < / > } } pub const LUCIDE_SCALING : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;