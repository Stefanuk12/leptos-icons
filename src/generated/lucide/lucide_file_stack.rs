use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 7h-3a2 2 0 0 1-2-2V2" ></ path > < path d = "M21 6v6.5c0 .8-.7 1.5-1.5 1.5h-7c-.8 0-1.5-.7-1.5-1.5v-9c0-.8.7-1.5 1.5-1.5H17Z" ></ path > < path d = "M7 8v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H15" ></ path > < path d = "M3 12v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H11" ></ path > < / > } } pub const LUCIDE_FILE_STACK : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;